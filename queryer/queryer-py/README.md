# 构建 Python 编译环境

创建 virtual env，然后用 `maturin develop` 构建：
```bash
cd queryer-py
cargo build

python -m venv .env
source .env/Scripts/activate

pip install maturin
maturin develop
```

之后可以使用：

```ipython
python
In [1]: import queryer_py
In [2]: sql = queryer_py.example_sql()
In [3]: print(queryer_py.query(sql))
```
