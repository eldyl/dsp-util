# dsd-util 

This is a simple tool for my homelab that I use with [docker-stack-deploy](https://github.com/wez/docker-stack-deploy).

```bash
A simple helper for managing your docker-stack-deploy containers.

Usage: dsd-util <COMMAND>

Commands:
  init     Initialize and bootstrap a new instance of docker-stack-deploy
  logs     View container logs
  nuke     Kill all docker containers and redeploy docker-stack-deploy
  restart  Restart containers
  stats    View basic stats for docker containers
  update   Update container images
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
```

## TODO

- [ ] Improve docs
