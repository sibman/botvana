//! Trading engine

use crate::prelude::*;

/// Trading engine
pub struct TradingEngine {
    market_data_rx: spsc_queue::Consumer<MarketEvent>,
    indicator_rx: spsc_queue::Consumer<IndicatorEvent>,
}

impl TradingEngine {
    pub fn new(
        market_data_rx: spsc_queue::Consumer<MarketEvent>,
        indicator_rx: spsc_queue::Consumer<IndicatorEvent>,
    ) -> Self {
        Self {
            market_data_rx,
            indicator_rx,
        }
    }
}

#[async_trait(?Send)]
impl Engine for TradingEngine {
    type Data = ();

    async fn start(mut self, shutdown: Shutdown) -> Result<(), EngineError> {
        info!("Starting trading engine");

        run_trading_loop(self.market_data_rx, self.indicator_rx, shutdown).await
    }

    /// Returns dummy data receiver
    fn data_rx(&mut self) -> spsc_queue::Consumer<Self::Data> {
        let (_data_tx, data_rx) = spsc_queue::make::<()>(1024);
        data_rx
    }
}

impl ToString for TradingEngine {
    fn to_string(&self) -> String {
        "trading-engine".to_string()
    }
}

/// Trading engine loop
pub async fn run_trading_loop(
    market_data_rx: spsc_queue::Consumer<MarketEvent>,
    indicator_rx: spsc_queue::Consumer<IndicatorEvent>,
    _shutdown: Shutdown,
) -> Result<(), EngineError> {
    loop {
        if let Some(_event) = market_data_rx.try_pop() {
            //info!("market data = {:?}", event);
        }

        if let Some(event) = indicator_rx.try_pop() {
            info!("indicator = {:?}", event);
        }
    }
}
