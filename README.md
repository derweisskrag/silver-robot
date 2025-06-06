# My toolkit


> Please note that it will be updated soon. Since the Spring Semester has ended, I have learned a large number of new features.



## How to run Rust code:

First of all, please make sure you have Rust downloaded:

    - rustc
    - cargo
    - rustup

Please, visit: https://www.rust-lang.org/tools/install

## How do you run the code?

### Using `Cargo`

This is the straightforward approach:

```
cargo run
```

It will run the code as long as they are valid syntax. On my machine, at 22:50 on the 6th of June in 2025, it successfully ran. I used the latest Rust version, which is

```
rustup -V
```

Showed:


```
rustc 1.86.0 (05f9846f8 2025-03-31)
```

Because I did not have any tests - it is not any app. It is just code for you to run them if you are not sure, and for me to expand on later, as I grow with Rust. 

## Rust-inspired code:

> It is not finished. However, it does contain the code I showcased here:

You can download the package (Python): [here](https://test.pypi.org/project/tomodachi/)

```py
from functools import cached_property
from tomodachi_core.common_types.result import Result, Ok, Err, result_wrapper
import pandas as pd


class Service:
    def __init__(self, path: str) -> None:
        if not isinstance(path, str):
            raise ValueError("Please, provide valid path!")
        
        self.path = path


    def load_csv_data(self) -> Result[pd.DataFrame, str]:
        try:
            df = pd.read_csv(self.path)

            self.df = (
                Ok(df)
                .and_then(self.remove_unnamed_and_convert_dates)
                .and_then(self.remove_millimeters)
                .unwrap()
            )

            return Ok(self.df)
        except (
            FileNotFoundError, 
            pd.errors.ParserError,
              pd.errors.EmptyDataError, 
              Exception
              ) as e:
            return Err(e)
```

which then helps us

```py
class PandasService(Service):
    def __init__(self, path: str = "") -> None:
        super().__init__(path)
        self.preprocessor = PreprocessData()
        

    @override
    def load_csv_data(self):
        match super().load_csv_data():
            case Ok(value=df) if isinstance(df, pd.DataFrame):
                return df
            case Err(error=error):
                return Err(error=error)
```
