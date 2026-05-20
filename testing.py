import asyncio
import aiohttp
import random
import time

# --- Configuration ---
TOTAL_REQUESTS = 100000            # Total players to simulate
MAX_CONCURRENT_CONNECTIONS = 10000  # How many requests to fire at the exact same time
URL = "http://localhost:3000/join" # Your Rust server endpoint

async def send_player(session, i, semaphore):
    # The Semaphore prevents Python from crashing your OS network ports
    async with semaphore:
        payload = {
            "id": f"PythonTestPlayer_{i}",
            "mmr": random.randint(200, 3000)
        }
        try:
            # Send the JSON payload to the Rust server
            async with session.post(URL, json=payload) as response:
                await response.text()
        except Exception as e:
            # In massive load tests, it is normal for a few connections to drop
            pass 

async def main():
    semaphore = asyncio.Semaphore(MAX_CONCURRENT_CONNECTIONS)
    
    print(f"Starting load test: Sending {TOTAL_REQUESTS} requests to Rust...")
    start_time = time.time()
    
    # Use a single connection session for maximum throughput
    async with aiohttp.ClientSession() as session:
        # Create a massive list of tasks
        tasks = [send_player(session, i, semaphore) for i in range(TOTAL_REQUESTS)]
        
        # Fire them all off concurrently
        await asyncio.gather(*tasks)
        
    duration = time.time() - start_time
    print(f"Finished sending in {duration:.2f} seconds!")
    print(f"Throughput: {TOTAL_REQUESTS / duration:.0f} requests per second")

if __name__ == "__main__":
    asyncio.run(main())