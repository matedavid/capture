capture
---

Capture code snippets from the terminal to share or bookmark them.

### Currently supported languages
* Rust
* Python
* Javascript/Typescript
* Golang
* C/C++

### Usage examples
#### Create a new snippet
```sh
# Create from function
capture add name_of_bookmark --file index.js (--no-comments) function jsFunction

# Create from line interval
capture add name_of_bookmark --file index.js interval 5:15
```

#### Get existing snippet
```sh
capture get jsFunction
```

#### List saved snippets 
```sh
capture list (--oneline)
```

#### Delete a snippet
```sh
capture delete jsFunction
```
