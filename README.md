# <p align="center">Senvy</p>
<p align="center">server and a cli tool for managing environment variables for your projects on a local server</p>

#
## General info
Project is a simple solution to provide a local way to share env vars between machines and potentially having multiple versions of them for a single project. There is no control over who can make create/update/delete entries on the server.  
Term 'entry' used throughout the project refers to an entry on the server with a unique name which represents a single set of vars.

## Server
Installing directly (`cargo install --path ./sever/`) or by using Dockerfile  
Server defaults to port 8080 unless PORT var is set.

## CLI
Installing (installed under name 'senvy'): `cargo install --path ./cli/`

CLI relies on a '.senvy' file in the project for the information  
Commands
- **init** \<project name\> \<path the file with env vars\> \<server url\>  
    initialize senvy in the current working directory and creates an entry on the server with the provided information

- **new** \<project name\> \<path the file with env vars\> \<server url\>  
    same as init only it does not create a local config

- **delete** \<project name\>(opt) \<server url\>(opt)  
    deletes entry on the sever and optionally local config if it exists  
    arguments not provided are pulled from the local config

- **pull** \<project name\>(opt) \<server url\>(opt)  
    pulls vars from the server and creates/updates the local config  
    arguments not provided are pulled from the local config

- **push** \<project name\>(opt) \<path the file with env vars\>(opt) \<server url\>(opt)  
    updates entry on the server if entry with the given name exists   
    arguments not provided are pulled from the local config

- **check**  
    check if there are more recent vars available for the current project and optionally update local config