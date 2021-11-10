# queryer-js

构建：

```bash
yarn
```

然后打开 node，可以测试：

```node
node
> const q = require('.')
> const sql = q.example_sql()
> sql
> console.log(q.query(sql))
```
