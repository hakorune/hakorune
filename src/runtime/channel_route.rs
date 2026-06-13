//! Await-visible route descriptors for future canonical `Channel<T>` calls.
//!
//! This is a fail-fast bridge for `CONC-CHANNEL-003`. It records which source
//! shapes are valid without opening MIR lowering or ordinary blocking calls.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HakoChannelRoute {
    AwaitSend,
    AwaitRecv,
    AwaitClose,
    TrySend,
    TryRecv,
}

impl HakoChannelRoute {
    pub const fn key(self) -> &'static str {
        match self {
            Self::AwaitSend => "hako.channel.await_send",
            Self::AwaitRecv => "hako.channel.await_recv",
            Self::AwaitClose => "hako.channel.await_close",
            Self::TrySend => "hako.channel.try_send",
            Self::TryRecv => "hako.channel.try_recv",
        }
    }

    pub const fn report_field(self) -> &'static str {
        match self {
            Self::AwaitSend => "channel_route_await_send_descriptor_present",
            Self::AwaitRecv => "channel_route_await_recv_descriptor_present",
            Self::AwaitClose => "channel_route_await_close_descriptor_present",
            Self::TrySend => "channel_route_try_send_descriptor_present",
            Self::TryRecv => "channel_route_try_recv_descriptor_present",
        }
    }

    pub const fn method_name(self) -> &'static str {
        match self {
            Self::AwaitSend | Self::TrySend => "send",
            Self::AwaitRecv | Self::TryRecv => "recv",
            Self::AwaitClose => "close",
        }
    }

    pub const fn requires_await(self) -> bool {
        matches!(self, Self::AwaitSend | Self::AwaitRecv | Self::AwaitClose)
    }

    pub const fn source_shape(self) -> &'static str {
        match self {
            Self::AwaitSend => "await ch.send(value)",
            Self::AwaitRecv => "await ch.recv()",
            Self::AwaitClose => "await ch.close()",
            Self::TrySend => "ch.try_send(value)",
            Self::TryRecv => "ch.try_recv()",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelRouteDescriptor {
    pub route: HakoChannelRoute,
    pub key: &'static str,
    pub report_field: &'static str,
    pub method_name: &'static str,
    pub source_shape: &'static str,
    pub requires_await: bool,
}

impl ChannelRouteDescriptor {
    pub const fn new(route: HakoChannelRoute) -> Self {
        Self {
            route,
            key: route.key(),
            report_field: route.report_field(),
            method_name: route.method_name(),
            source_shape: route.source_shape(),
            requires_await: route.requires_await(),
        }
    }
}

pub const CHANNEL_ROUTE_DESCRIPTORS: &[ChannelRouteDescriptor] = &[
    ChannelRouteDescriptor::new(HakoChannelRoute::AwaitSend),
    ChannelRouteDescriptor::new(HakoChannelRoute::AwaitRecv),
    ChannelRouteDescriptor::new(HakoChannelRoute::AwaitClose),
    ChannelRouteDescriptor::new(HakoChannelRoute::TrySend),
    ChannelRouteDescriptor::new(HakoChannelRoute::TryRecv),
];

pub fn channel_route_descriptors() -> &'static [ChannelRouteDescriptor] {
    CHANNEL_ROUTE_DESCRIPTORS
}

pub fn channel_route_report_fields() -> Vec<(&'static str, &'static str)> {
    CHANNEL_ROUTE_DESCRIPTORS
        .iter()
        .map(|descriptor| (descriptor.report_field, "1"))
        .collect()
}

pub fn channel_route_activation_report_fields() -> Vec<(&'static str, &'static str)> {
    vec![
        ("channel_route_hidden_blocking_ordinary_call_enabled", "0"),
        ("channel_route_mir_lowering_enabled", "0"),
        ("channel_route_program_json_enabled", "0"),
        ("channel_route_llvm_enabled", "0"),
        ("channel_route_legacy_p2p_channelbox_reused", "0"),
    ]
}

pub fn channel_route_source_shape_report_fields() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "channel_route_await_send_source_shape",
            "await ch.send(value)",
        ),
        ("channel_route_await_recv_source_shape", "await ch.recv()"),
        ("channel_route_await_close_source_shape", "await ch.close()"),
        ("channel_route_try_send_source_shape", "ch.try_send(value)"),
        ("channel_route_try_recv_source_shape", "ch.try_recv()"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_route_keys_are_stable() {
        let keys: Vec<_> = channel_route_descriptors()
            .iter()
            .map(|descriptor| descriptor.key)
            .collect();

        assert_eq!(
            keys,
            vec![
                "hako.channel.await_send",
                "hako.channel.await_recv",
                "hako.channel.await_close",
                "hako.channel.try_send",
                "hako.channel.try_recv",
            ]
        );
    }

    #[test]
    fn channel_route_await_requirement_is_stable() {
        let await_routes: Vec<_> = channel_route_descriptors()
            .iter()
            .filter(|descriptor| descriptor.requires_await)
            .map(|descriptor| descriptor.source_shape)
            .collect();

        assert_eq!(
            await_routes,
            vec![
                "await ch.send(value)",
                "await ch.recv()",
                "await ch.close()"
            ]
        );
    }

    #[test]
    fn channel_route_activation_keeps_lowering_closed() {
        assert_eq!(
            channel_route_activation_report_fields(),
            vec![
                ("channel_route_hidden_blocking_ordinary_call_enabled", "0"),
                ("channel_route_mir_lowering_enabled", "0"),
                ("channel_route_program_json_enabled", "0"),
                ("channel_route_llvm_enabled", "0"),
                ("channel_route_legacy_p2p_channelbox_reused", "0"),
            ]
        );
    }
}
