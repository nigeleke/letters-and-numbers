# Letters and Numbers - Rust implementation

## Build
```
trunk build --release
docker-compose build
```
## Note

1. The <code>Backend</code> isn't _really_ a backend. The folder is named as such simply to structure the implementation.

## Todo

1. Ideally the <code>ResolverService Agent</code> should execute in a non-UI thread. My current understanding of Yew <code>Agent</code>'s
   is that the <code>Reach</code> should be <code>Public</code> rather than <code>Context</code>.
   This is currently failing & under investigation.
