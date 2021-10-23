# LMC preprocessor
A simple project which (currently) adds simple macro support to [LMC](https://mjrbordewich.webspace.durham.ac.uk/wp-content/uploads/sites/186/2021/04/LMC-Instruction-Set.pdf). As an example:
```
IN
STO a
IN
STO b
IN
STO c
```
could be written as 
```
macro IN_STO(location) = {
    IN
    STO location
}

IN_STO!(a)
IN_STO!(b)
IN_STO!(c)
```

## Usage
* Piped data: `echo "ADD 10" | ./lmc-preprocessor`
* Data from a file: `./lmc-preprocessor reference.asm`