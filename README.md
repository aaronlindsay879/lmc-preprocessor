# LMC preprocessor
A simple project which (currently) adds simple macro support to LMC. As an example:
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
IN_STO(location) = {
    IN
    STO location
}

IN_STO!(a)
IN_STO!(b)
IN_STO!(c)
```