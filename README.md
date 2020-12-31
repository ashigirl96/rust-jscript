# jscript

just extract `"scripts"` from package.json

## Example

package.json

```json
{
  "name": "with-typescript",
  "version": "1.0.0",
  "scripts": {
    "dev": "next",
    "build": "next build",
    "start": "next start",
    "type-check": "tsc"
  },
  "dependencies": {
    "next": "latest",
    "react": "^16.12.0",
    "react-dom": "^16.12.0"
  },
  "devDependencies": {
    "@types/node": "^12.12.21",
    "@types/react": "^16.9.16",
    "@types/react-dom": "^16.9.4",
    "typescript": "4.0"
  },
  "license": "MIT"
}

```

run command

```bash
➜ jscript
build:          next build
dev:            next
start:          next start
type-check:     tsc
```



