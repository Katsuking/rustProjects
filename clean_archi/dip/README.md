# rustで 疎結合ってどうやんの?

DIP(dependency Inversion Principal)

rustにおけるclean architectureってどうやってやるんだ?
まじで正解だとは思ってないけど、とりあえず...

### ポイント

・BusinessLogicはDataProviderのtraitに依存している

-> FileDataProviderやDatabaseDataProviderの実際の処理はDataProviderのtraitさえ実装していればあとはどうでもいい

```mermaid
graph TD
    %% モジュールの定義
    subgraph "src"
        subgraph "my_module"
            A[DataProvider] 
            B[FileDataProvider] 
            C[DatabaseDataProvider] 
            D[BusinessLogic]
        end
        E[main.rs]
    end

    %% ノード間の依存関係を定義
    A --> B
    A --> C

    %% ビジネスロジックの依存関係を定義
    D --> A
```

全体の処理としては以下のような流れ

```mermaid
graph TD
    %% モジュールの定義
    subgraph "src"
        subgraph "モジュール"
            A[DataProvider]
            B[FileDataProvider]
            C[DatabaseDataProvider]
            D[BusinessLogic]
        end
        E[main.rs]
    end

    %% ノード間の依存関係
    A --> B
    A --> C
    D --> A

    %% 処理の流れ
    E -->|Uses| D
    D -->|Uses| B
    D -->|Uses| C
```

