use lotus_client_rs::blockanalyzer::{Message,iterate_over_blockchain};
use lotus_client_rs::cbor::deal_proposal::decode_storage_deal;

use env_logger;
use log;
mod cli;
mod db;

fn main() {
    env_logger::Builder::from_default_env().format_timestamp(None).init();

    // Get CLI and config.toml args
    let exec_params = cli::parse_configuration();

    // Set up API object and test connection
    let api = lotus_client_rs::api::ApiClient::new(&exec_params.endpoint);
    assert!(api.check_endpoint_connection());
    
    // Define the callbacks
    let on_start_new_tipset = |height:u64,_blocks:&Vec<String>| {
        // add a row to tipset_processing_status table (cols: tipset_height, reported_starting, reported_ending)
        println!("Tipset: {}", height);
    };
    let on_complete_tipset = |height:u64| {
        // update tipset_processing_status.reported_ending
        println!("Completed tipset: {}", height);
    };

    let on_start_new_block = |blkcid:&str| {
        println!("  Block: {}", blkcid);
    };

    let on_found_new_message_cid = |_msg_cid:&str| {
        // No-op
        log::info!("cid = '{}'",_msg_cid);
    };

    // Search for deal proposals (with Payload CID <-> Piece CID mappings)
    let on_found_new_message = |msg_cid:&str, msg:&Message| {
        log::info!("cid = '{}'\n\nmsg={:?}",msg_cid,msg);

        if msg.method=="4" && msg.to=="t05" {
            println!("Found a dealproposal:");
            println!("    cid = '{}'\n    msg={:?}\n",msg_cid,msg);
            if let Some(decoded_params) = decode_storage_deal(&msg.params) {
                println!("    decoded params = '{:?}'\n",decoded_params);
                println!("        piece_cid as str = '{}'\n",decoded_params.get_piece_cid_as_str());
    
                // Persist tuple to database
                let conn = db::get_db_connection();
                db::insert_piece_and_payload_cids(&conn,msg_cid,&decoded_params.get_piece_cid_as_str(),&decoded_params.label);
            } else {
                log::error!("on_found_new_message:  could not decode params for {}",msg_cid);
            }
        }
    };

    // Run executor with our callbacks
    iterate_over_blockchain(exec_params.min, exec_params.max, &api, 
        Some(on_start_new_tipset),
        Some(on_start_new_block),
        Some(on_found_new_message_cid),
        Some(on_found_new_message),
        None,
        Some(on_complete_tipset)
    );
}