# RisingWave Overview

## Links
- [RisingWave](https://risingwave.io)
- [State Management](https://blog.zhuangty.com/state-management-in-risingwave/)
- [Materialized View](https://blog.zhuangty.com/napa)


## Outline
- RisingWave uses a storage repository called Hummock to manage the storage of internal state and materialized views.
- Hummock provides an interface similar to a Key-Value store.
- RisingWave is based on a fixed epoch-based state management mechanism.
- Upon receiving a barrier, operators will synchronize data to Hummock and reset local state.
- Shared Buffer is introduced to address issues with asynchronous data uploads and checkpoint success.
- RisingWave has designed a cloud-based large LSM tree structure for managing state storage.
- Compaction tasks are used for garbage collection and data organization, which can be flexibly scheduled according to requirements. 
- RisingWave continues its iterative process and has introduced Shared State to reduce storage states.

## Conclusion
![trade-off](https://user-images.githubusercontent.com/9161438/166149580-65a119cf-5071-42ae-89cb-443686365df7.png)
