use crate::{client::Client, errors::Error, RunParameters};

const STATE_INITIALIZED_GLOBAL: &str = "initialized_global";
// const STATE_INITIALIZED_GROUP_FMT: &str = "initialized_group_%s";

pub async fn init(
    client: Client,
    params: RunParameters,
) -> Result<InitContext, Box<dyn std::error::Error>> {
    let group_state = format!("initialized_group_{}", params.test_group_id);

    // client.record_message("waiting for network to initialize");
    client.wait_network_initialized().await?;
    // client.record_message("network initialization complete");

    // NOTE: in go we're using i64.
    let global_seq: u64 = client.signal(STATE_INITIALIZED_GLOBAL).await?;
    let group_seq: u64 = client.signal(group_state).await?;

    // client.record_message("claimed sequence numbers; global=%d, group(%s)=%d", global_seq, test_group_id, group_seq);

    let ic = InitContext {
        client,
        params,
        global_seq,
        group_seq,
    };

    Ok(ic)
}

// TODO: drop / close?

pub struct InitContext {
    pub client: Client,
    pub params: RunParameters,
    pub global_seq: u64,
    pub group_seq: u64,
}

impl InitContext {
    pub async fn wait_all_instances_initialized(&self) -> Result<(), Error> {
        self.client
            .barrier(STATE_INITIALIZED_GLOBAL, self.params.test_instance_count)
            .await?;
        Ok(())
    }
    pub async fn wait_group_instances_initialized(&self) -> Result<(), Error> {
        let group_state = format!("initialized_group_{}", self.params.test_group_id);
        self.client
            .barrier(group_state, self.params.test_group_instance_count)
            .await?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn io_test() {
//         // //let raw_response = r#"{"key": "run:c7uji38e5te2b9t464v0:plan:streaming_test:case:quickstart:run_events", "error": "failed to decode as type *runtime.Event: \"{\\\"stage_end_event\\\":{\\\"name\\\":\\\"network-initialized\\\",\\\"group\\\":\\\"single\\\"}}\"", "id": "0"}"#;

//         // let event = Event {
//         //     event: EventType::StageStart {
//         //         name: "network-initialized".to_owned(),
//         //         group: "single".to_owned(),
//         //     },
//         // };

//         // let json = serde_json::to_string(&event).unwrap();

//         // println!("{:?}", json);
//     }
// }
