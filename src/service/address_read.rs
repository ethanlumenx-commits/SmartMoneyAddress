
use crate::service::swaps_seriver::SwapsService;
use crate::{helius::client::HeliusClient};
use crate::config::load_config;
use crate::helius::enhanced::{EnhancedClient};
use crate::repo::raw_addresses_repo::RawAddressesRepo;
use crate::service::helius_json_seriver::HleiusJsonService;

use sqlx::PgPool;

use tracing::{info, warn, error, debug};

struct AddressReadService;


impl AddressReadService {
    /// find address that are not checked in database raw_addresses table and insert them into swaps table
    pub async fn find_by_address(db: &PgPool) -> anyhow::Result<()> {
        info!("Starting AddressReadService::find_by_address");
        let config = load_config();
        let helius_client = HeliusClient::new(config.helius_api_url);
        let enhanced_client = EnhancedClient::new(config.helius_enhanced_api_url);

        loop {
            let result = RawAddressesRepo::get_wait_check_addresses(db, 20).await?;
            let count = result.len();
            debug!("Fetched {} wait-check addresses from database", count);

            if result.is_empty() {
                info!("No more wait-check addresses, exiting loop");
                break;
            }

            info!("Processing {} addresses in current batch", count);
            let mut processed_count = 0;
            let mut inserted_count = 0;

            for item in result.iter() {
                info!("Processing address: {}", item.address);

                let helius_result = match helius_client.get_signatures_for_address(&item.address, 100).await {
                    Ok(v) => {
                        debug!("Fetched {} signatures for address {}", v.len(), item.address);
                        v
                    },
                    Err(e) => {
                        warn!("Skipping address {}: failed to get signatures - {}", item.address, e);
                        continue;
                    }
                };

                let signatures = helius_result.iter().map(|x| x.signature.clone()).collect::<Vec<_>>();
                debug!("Extracted {} signatures for address {}", signatures.len(), item.address);

                let enhanced_result = match enhanced_client.get_transactions_for_signatures(signatures).await {
                    Ok(v) => {
                        debug!("Fetched {} transactions for address {}", v.len(), item.address);
                        v
                    },
                    Err(e) => {
                        warn!("Skipping address {}: failed to get transactions - {}", item.address, e);
                        continue;
                    }
                };

                let helius_json: Vec<_> = enhanced_result.iter().map(|x| x.to_db_model()).collect();
                debug!("Converted {} transactions to HeliusJson models for address {}", helius_json.len(), item.address);

                let filtered_json = SwapsService::filter_swaps(helius_json);
                let filtered_count = filtered_json.len();
                debug!("Filtered to {} swap transactions for address {}", filtered_count, item.address);

                if filtered_json.is_empty() {
                    info!("No swap transactions found for address {}, skipping insert", item.address);
                    processed_count += 1;
                    continue;
                }

                if let Err(e) = HleiusJsonService::batch_insert(db, filtered_json).await {
                    error!("Failed to insert data for address {}: {}", item.address, e);
                    continue;
                }

                inserted_count += filtered_count;
                processed_count += 1;
                info!("Successfully inserted {} swap transactions for address {}", filtered_count, item.address);
            }

            info!("Batch completed: processed {} addresses, inserted {} swap transactions", processed_count, inserted_count);

            let filter_result = result.iter().map(|x| x.address.clone()).collect::<Vec<_>>();
            let filter_count = filter_result.len();
            RawAddressesRepo::update_wait_check_addresses(db, filter_result).await?;
            info!("Updated {} addresses to checked status", filter_count);
        }

        info!("AddressReadService::find_by_address completed successfully");
        Ok(())
    }

    /// find address that are not checked by address
    pub async fn find_by_address_by_address(db: &PgPool, address: &str) -> anyhow::Result<()> {
        info!("Starting AddressReadService::find_by_address_by_address");
        let config = load_config();
        let helius_client = HeliusClient::new(config.helius_api_url);
        let enhanced_client = EnhancedClient::new(config.helius_enhanced_api_url);

        let helius_result = match helius_client.get_signatures_for_address(address, 100).await {
            Ok(v) => {
                debug!("Fetched {} signatures for address {}", v.len(), address);
                v
            },
            Err(e) => {
                warn!("Skipping address {}: failed to get signatures - {}", address, e);
                return Ok(())
            }
        };

        let signatures = helius_result.iter().map(|x| x.signature.clone()).collect::<Vec<_>>();

        let enhanced_result = match enhanced_client.get_transactions_for_signatures(signatures).await {
            Ok(v) => {
                debug!("Fetched {} transactions for address {}", v.len(), address);
                v
            },
            Err(e) => {
                warn!("Skipping address {}: failed to get transactions - {}", address, e);
                return Ok(())
            }
        };

        let helius_json: Vec<_> = enhanced_result.iter().map(|x| x.to_db_model()).collect();
        debug!("Converted {} transactions to HeliusJson models for address {}", helius_json.len(), address);
        let filtered_json = SwapsService::filter_swaps(helius_json);
        let filtered_count = filtered_json.len();
        debug!("Filtered to {} swap transactions for address {}", filtered_count, address);
        if filtered_json.is_empty() {
            info!("No swap transactions found for address {}, skipping insert", address);
            return Ok(())
        }
        if let Err(e) = HleiusJsonService::batch_insert(db, filtered_json).await {
            error!("Failed to insert data for address {}: {}", address, e);
            return Ok(())
        }
        info!("Successfully inserted {} swap transactions for address {}", filtered_count, address);
        RawAddressesRepo::update_wait_check_addresses(db, vec![address.to_string()]).await?;
        info!("Updated {} addresses to checked status", 1);
        info!("AddressReadService::find_by_address_by_address completed successfully");

        Ok(())

    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::connect::connect;
    use crate::logger::init_logger;

    #[tokio::test]
    async fn test_find_by_address() {
        init_logger();
        info!("Starting test_find_by_address");
        
        let db = connect().await;
        let result = AddressReadService::find_by_address(&db).await;
        
        assert!(result.is_ok());
        info!("test_find_by_address completed successfully");
    }
}