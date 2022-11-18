# Local Money Fiat Prices Aggregator
This software aggregates the USD rate for the fiat currencies listed on Local Money, so we can calculate the price of tokens against regional fiat currencies and then derive the price of token/fiat using ATOM/USD from the Kujira Oracle as price reference.

It fetches the USD rate for currencies using 2 sources, which act as a fail-safe for each other, returning the average between both sources if both are available. Then, it sends a message to the Price contract of the protocol to store these fiat prices on-chain.

# Running
1) Clone the repo and navigate into it
2) Compile and print the cron command with `sh cron.sh`
3) Copy the cron command and add it to your crontab with `crontab -e`

