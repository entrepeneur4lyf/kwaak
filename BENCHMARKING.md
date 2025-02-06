Benchmarking
============

We highly encourage the use of benchmarks to measure and improve the performance of Kwaak.

Kwaak is supported by the agent test harness we've developed to compare agent performance across different benchmarks
and LLM's. You can find it [here](https://github.com/bosun-ai/agent-test-harness).

Specifically to run the SWE-bench using the test harness, run the following command:

```shell
git clone https://github.com/bosun-ai/agent-test-harness
cd agent-test-harness
cargo install amsterdam derrick
export OPENAI_API_KEY=YOUR_OPENAI_API_KEY
uv run agent-test-swe
```

## Debugging

The agent test harness intentionally does not clean up after each test run to allow you to inspect the state of the
agent container after running.

Run `docker ps` to see the containers and use `docker exec -it <container> bash` to enter the container. There will
be various files related to setting up and running the benchmark in the `/tmp` folder. Additionally there are the
Kwaak logs located in `/root/.cache/kwaak/logs/`. 

If you noticed an interesting failure, please file an issue and attach the contents of the logs and the `/tmp` folder to the issue. Note that there is a dump of the environment variables in the `/tmp/setup.log`. The OpenAI key in there is not an actual key, but an intermediary key supplied by the LLM proxy. If you have OTEL configured however its keys will be in this file so they should be removed manually.