# BadWifiCard
My wifi card for my server sucks. I cant use ethernet for other reasons, so here is my solution.

## Problem
Every 10ish days, my wifi card dies. No logs, no failed processes, just unable to connect to the internet. The only way to fix this is rebooting. restarting NetworkManager or wpa_supplicant yields nothing, and the real end goal is a new card or somehow ethernet. In the meantime, heres some god awful code i slapped together to "fix" it.

## Solution
I ping the internet. Incase cloudflare or something goes down, i have multiple domains i ping every 5 minutes. If all the domains fail to ping for two cycles in a row, then the server restarts. 
