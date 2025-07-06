https://github.com/kentcdodds/cross-env


```bash
cross-env GREET=Joe bash -c 'echo $GREET'
```

```json
{
  "scripts": {
    "build": "cross-env NODE_ENV=production webpack --config build/webpack.config.js"
  }
}
```