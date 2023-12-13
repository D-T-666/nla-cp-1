# Computational project #1

Encryption and decryption of `txt`/`wav` files using ordinary linear algebra.

---

## Handling `txt` files

### Encryption:

1. Read the file.
2. Convert the contents (ASCII characters) into 2-digit arithmetic by 
   splitting the 8 bits of ascii code into two - upper and lower parts
   giving two integers with values in [0, 15].
3. Cast the integeres into floats.
4. Reshape the vector of obtained floats into (nxm) matrix, where n is 
   the size of the key matrix and m is ceil(len(data)/n). An (nxm) matrix
   is the smallest matrix with n rows which can fit all elements.
5. Multiply the key matrix with the obtained matrix to get the encrypted
   matrix E (elements of which are floating point numbers).
6. Convert the matrix of floats into a vector of... floats.
7. Chop up the floats into 4-bit nibbles.
8. Again, each will have values  in [0, 15] so add the ASCII code of the
   character 'a' to every one of them giving us the ASCII codes for ['a', 'p']
9. Store the vector of chars in the output file. Also include the length
   of the initial message as a header in the text file.

### Decryption:

1. Read the file.
2. Convert the contents (ASCII characters) into 2-digit arithmetic by 
   subtracting the ASCII code of the character 'a' from every one of them.
   This is valid since we store only the characters ['a', 'p'].
3. The bits resulting vector of numbers should be concatenated in batches 
   of 8, giving us 32-bit numbers.
4. Cast the 32-bit numbers into floating point numbers giving us the numbers 
   which we encrypted.
5. Convert the vector of floating point numbers into a matrix with n rows, 
   where n is the number of rows of the key martix.
6. For each column b, do either:
    - find the solution to the system of equations (L+I)(U+I)x = b via Thomas'
      algorithm.
    - multiply by the inverse of the K matrix x = ((L+I)(U+I))^{-1}b.
7. Convert the resulting matrix of floats into a vector of integers by rounding.
8. Concatenate the bits of paris of 4-bit numbers to form 8-bit numbers. This is
   possible because the results of step 7 should be in [0, 15].
9. Truncate the vector of chars to the length provided in the header of the file.
10. Write the resulting vector of ASCII codes to the output file as characters. 

## Handling `wav` files

### Encryption:

1. Read the file.
2. Convert the contents (16-bit integers) into 2-digit arithmetic by 
   splitting the 16 bits into four parts giving integers with values
   in [0, 15].
3. Cast the integeres into floats.
4. Reshape the vector of obtained floats into (nxm) matrix, where n is 
   the size of the key matrix and m is ceil(len(data)/n). An (nxm) matrix
   is the smallest matrix with n rows which can fit all elements.
5. Multiply the key matrix with the obtained matrix to get the encrypted
   matrix E (elements of which are floating point numbers).
6. Convert the matrix of floats into a vector of floats.
7. Chop up the bits of floats into 16-bit integers.
8. Prepend the vector with two 16-bit numbers - halves of a 32-bit number 
   equal to the length of the data.
8. Store the vector of 16-bit integers in the output file.

### Decryption:

1. Read the file.
2. Concatentate pairs of 16-bit integers into 32-bit integers.
3. Convert the vector of 32-bit integers into a matrix of floats by casting.
4. For each column b, do either:
    - find the solution to the system of equations (L+I)(U+I)x = b via Thomas'
      algorithm.
    - multiply by the inverse of the K matrix x = ((L+I)(U+I))^{-1}b.
5. Convert the resulting matrix of floats into a vector of integers by rounding.
6. Concatenate the bits of paris of 4-bit numbers to form 16-bit numbers. This is
   possible because the results of step 5 should be in [0, 15].
7. Truncate the vector of chars to the length provided in the header of the file.
8. Write the resulting vector of 16-bit integers to the output file as characters. 

## Key matrix generation.

To generate a (nxn) key matrix, I do the following:

1. Generate a (nxn) matrix with random floats in range [0, 1/n] as elements.
2. Set the diagonal entries to 1.
3. [optional] if matrix needs to have integer entries, do the following:
    1. Multiply every element by 10 * n.
    2. Set the diagonal entries to n.

---

## Conclusion.

This method of encryption, although being very inneficient and insecure in almost
every way imaginable, is quite easy to implement. Not only that, the error is almost
non-existent after decryption!

- The data can be recovered with 100% accuracy if the direct method is used. This
  is bevause of the matrix chosen.
- My method (and the implementation) is not fast enough for my taste. I think that such
  symmetric encription algorithms should be much faster, as the industry standards are.
- This implementation requires O(n^2) memory to work.
- Applying this method would only make sense for text and audio data, either in a chat
  application or for encrypting your private files.
- This method has an advantage of being fast. There's no iterations for finding the key 
  matrix.
  
---

## Running the program.

First, make sure you have `cargo` installed, if you don't you can start with [rustup](https://rustup.rs/).

After you're all done installing rust, `cd` into the directory of this project and run

```
cargo install --path .
```

This will install a binary named `bzit`. This is the program.

#### Generate a key.

The command used to generate key matrices is the following:

```
bzit gen-key [OPTIONS] --key-path <KEY_PATH> --chunk-size <CHUNK_SIZE> 
```

#### Encrypt a file.

```
bzit encrypt --key-path <KEY_PATH> --file-path <FILE_PATH> 
```

#### Decrypt a file.

```
bzit decrypt-direct --key-path <KEY_PATH> --file-path <FILE_PATH> 
```

```
bzit decrypt-iterative [OPTIONS] --key-path <KEY_PATH> --file-path <FILE_PATH> 
```

#### Help.

```
bzit help
```

### windows

`bzit.exe` can be found in `target/x86_64-pc-windows-gnu/release`.