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

run command: no option

```bash
➜ jscript
build:          next build
dev:            next
start:          next start
type-check:     tsc
```

run command: specific package.json

```
➜ jscript ../electron-playground/package.json
build:          npm run build-renderer && npm run build-electron
build-electron: tsc -p electron-src
build-renderer: next build renderer && next export renderer
clean:          rimraf dist main renderer/out renderer/.next
console:        hasura console --project hasura
dev:            npm run build-electron && electron .
dist:           npm run build && electron-builder
fix:            yarn eslint --fix renderer
gql-gen.next:   yarn gql-gen -c renderer/codegen.yml
pack-app:       npm run build && electron-builder --dir
test:           jest --config ./renderer/jest.config.js
type-check:     tsc
```


