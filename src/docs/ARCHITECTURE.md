```mermaid
graph LR
    A[ExchangeWrapper] -- Private API --> B{Simulated OMS[Exchange,Pair]}
    A -- Public API --> C((API transformer))
    C --> D{External Exchanges}

graph sequenceDiagram
    BBGO->>ExchangeWrapper: Private/Public Subscribe(formatted)
    ExchangeWrapper->>External Exchange: Private/Public Subscribe(transformed)
    loop Event passing
        External Exchange->>ExchangeWrapper: Public/Private messages
        ExchangeWrapper->>BBGO: Public/Private messages(formatted)
    end
```
