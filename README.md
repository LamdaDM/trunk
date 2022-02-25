# Description
**Trunk** is a personal utility TCP server for external logging and hashing, made using **Bunker**. Logs are saved to a MySQL database and hashes are generated using Argon2.


### TODO
- [X] Load config through cfg_loader.
- [ ] When IO-related errors occur when persisting logs, store to a local file and retry later.
- [X] Rework messaging protocol.
- [ ] Allow trunk to run without non-essential services, and track active and inactive services.
- [ ] Revisit persistence implementation for logs.