// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{LoadDestination, NetworkLoadTest};
use aptos_forge::{
    GroupNetworkBandwidth, NetworkContext, NetworkTest, Swarm, SwarmChaos, SwarmNetworkBandwidth,
    Test,
};

pub struct NetworkBandwidthTest;

// Bandwidth
// Indicates the rate of bandwidth limit
pub const RATE_MBPS: u64 = 100;
// Indicates the number of bytes waiting in queue
pub const LIMIT_BYTES: u64 = 20971520;
// Indicates the maximum number of bytes that can be sent instantaneously
pub const BUFFER_BYTES: u64 = 10000;

impl Test for NetworkBandwidthTest {
    fn name(&self) -> &'static str {
        "network::bandwidth-test"
    }
}

impl NetworkLoadTest for NetworkBandwidthTest {
    fn setup(&self, ctx: &mut NetworkContext) -> anyhow::Result<LoadDestination> {
        let all_validators = ctx
            .swarm()
            .validators()
            .map(|v| v.peer_id())
            .collect::<Vec<_>>();
        ctx.swarm()
            .inject_chaos(SwarmChaos::Bandwidth(SwarmNetworkBandwidth {
                group_network_bandwidths: vec![GroupNetworkBandwidth {
                    name: format!("forge-namespace-{}mbps-bandwidth", RATE_MBPS),
                    rate: RATE_MBPS,
                    limit: LIMIT_BYTES,
                    buffer: BUFFER_BYTES,
                    source_nodes: all_validators.clone(),
                    target_nodes: all_validators,
                }],
            }))?;
        let msg = format!(
            "Limited bandwidth to {}mbps with limit {} and buffer {} to namespace",
            RATE_MBPS, LIMIT_BYTES, BUFFER_BYTES
        );
        println!("{}", msg);
        ctx.report.report_text(msg);
        Ok(LoadDestination::FullnodesOtherwiseValidators)
    }

    fn finish(&self, swarm: &mut dyn Swarm) -> anyhow::Result<()> {
        let all_validators = swarm.validators().map(|v| v.peer_id()).collect::<Vec<_>>();
        swarm.remove_chaos(SwarmChaos::Bandwidth(SwarmNetworkBandwidth {
            group_network_bandwidths: vec![GroupNetworkBandwidth {
                name: format!("forge-namespace-{}mbps-bandwidth", RATE_MBPS),
                rate: RATE_MBPS,
                limit: LIMIT_BYTES,
                buffer: BUFFER_BYTES,
                source_nodes: all_validators.clone(),
                target_nodes: all_validators,
            }],
        }))
    }
}

impl NetworkTest for NetworkBandwidthTest {
    fn run<'t>(&self, ctx: &mut NetworkContext<'t>) -> anyhow::Result<()> {
        <dyn NetworkLoadTest>::run(self, ctx)
    }
}
