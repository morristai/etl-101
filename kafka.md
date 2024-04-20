# Kafka

## Kafka Message Consistency

### Types of Consistency

- **At-Least-Once Delivery**: This is the default mode for Kafka producers. It guarantees that each message will be delivered at least once. There's potential for duplicates if retries are required due to failures.

- **At-Most-Once Delivery**: Messages may be lost but won't be duplicated. This happens if the producer does not retry on failure. It offers lower latency but less reliability.

- **Exactly-Once Delivery**: The ideal scenario, where each message is delivered and processed only once. This requires Kafka Streams, transactions, and idempotent producers and consumers.

### Producer Acknowledgments

- `acks=0`: Producer does not wait for any acknowledgment from the broker
- `acks=1`: Producer waits for the leader to acknowledge the message
- `acks=all`: Producer waits for the leader and all replicas to acknowledge the message

### Consumer Acknowledgments

- `auto-commit`: the consumer sends an acknowledgment to the broker as soon as it receives a message

- `after-processing`: the consumer only sends an acknowledgment to the broker after it has successfully processed the message

- `manual`: the consumer waits until it receives specific instructions before sending an acknowledgment to the broker

### In-Sync Replicas (ISR)

The number of `min.insync.replicas` for a topic influences availability and consistency.  If it's set to 2, at least the leader and one other replica must acknowledge a message before it's officially committed.

## Kafka Message Ordering

Kafka can't guarantee the order of messages **across partitions**.

- Specify partition: Kafka guarantees message ordering within a partition(FIFO), so messages with the same key will be stored in the same partition and processed in order.
  - Specify key (same partition, same as above)

## Kafka Idempotence 

- **Idempotent Producer**: Kafka also supports idempotent producers, which ensure that messages are sent exactly once by **assigning a unique identifier to each message**. This identifier allows Kafka to detect and discard duplicate messages, preventing them from being processed multiple times.

- **Transactional Producer**: Kafka provides transactional support for producers to ensure that messages are sent exactly once. By enabling transactional support, producers can send messages in a single transaction, guaranteeing that messages are either all sent or none are sent.

## Comsumer Pull vs. Push

- Kafka Listener is a pull-based approach
- Kafka Streams is a push-based approach

### Why Kafka Chose Pull

- Scalability: The pull model allows Kafka to easily scale to support very **large numbers of consumers** without the broker being burdened with individual consumer tracking.
- **Fault Tolerance**: Consumers can go offline and come back online, picking up where they left off. This prevents message loss and makes the system more robust.
- Flexibility: Developers have more fine-grained control over how consumers process Kafka messages, enabling use cases like batching and custom processing logic.

## Rebalancing in Kafka

Rebalancing is the process of redistributing partitions among **consumers in a consumer group** to ensure that each consumer receives a fair share of the workload.

## How does Kafka achieve zero-copy while persisting data to disk?

Kafka achieves both message persistence to disk and Zero Copy operations through complementary techniques. Message persistence ensures reliable storage by **asynchronously writing messages to disk after initially storing them in memory**. Zero Copy optimizes data transfer efficiency by minimizing memory copy operations during message transmission, such as directly passing memory addresses between components instead of copying message contents in memory. These techniques work together to provide reliable message storage while optimizing performance during message transfer in Kafka.