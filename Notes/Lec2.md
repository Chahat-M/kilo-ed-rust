### Doubts

- Is the last line appearing or not? Confused? Why last line appears in video and not in mine
- Step 40 - I used UntilNewLine, but isn't giving desired result.
- What and when to use saturating_add and saturating_sub ?
- Off screen not implemented correctly
- During PageUp and PageDown why move_to isn't working and not used ?

**Caution**
- Step 30 - 34 -> not needed so skipped
- Step 36 - 38 -> similar to refactoring/managing in separate files -> so skipped
- Step 40 -> not needed for now -> so skipped
- Step 54 -> not implemented until it is used

## Saturating_sub Use

It saturates at the value's minimum avoiding overflow.

Practically, if normal subtraction is performed then in the overflow case we get an error and the screen gets stucked. So to come out to the original mode, we will need to restart. On the other hand, if saturating_sub is used the overflow case is handled i.e no action is performed if overflow occurs.

But in the case of addition, both the ways give same output.

## Use of unwrap()

Unwrap is used to implicitly handle the cases i.e it will return the inner element or panic.

The function unwrap(self) -> T gives the embedded T if there is one with Result<T,E> and Option<T>. If instead there is not a T but an E (error) or None then it will panic.
