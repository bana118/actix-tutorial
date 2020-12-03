# actix-tutorial

actix-web，diesel.，teraを用いた例  

# How to get started
VSCodeの[Remote Container 拡張](https://code.visualstudio.com/docs/remote/containers)の使用を推奨．    
このプロジェクトをクローンし，VSCode の[Remote Container 拡張](https://code.visualstudio.com/docs/remote/containers)を使用して開く．  

```
git clone https://github.com/bana118/actix-tutorial.git
code actix-tutorial
```

データベースのセットアップ
```bash
diesel setup --database-url=database.sqlite3
diesel migration generate create_memos
```

作成されたup.sqlに以下を追記
```sql
CREATE TABLE memos (
  id INTEGER NOT NULL PRIMARY KEY,
  content VARCHAR NOT NULL
)
```

マイグレーション

```
diesel migration run --database-url=database.sqlite3
```

サーバー起動
```
cargo run
```

http://127.0.0.1:8080  
http://127.0.0.01:8080/form  
にアクセス

