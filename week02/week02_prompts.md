# Lab 02: Prompt Engineering Solutions

## Problem 1: Debugging

**My Prompt:**



You are a senior Python developer.

Context:
I have a Python function that should calculate the sum of all EVEN numbers
in a list of integers, but it is producing incorrect results.

Task:
Identify the logical bug in the function and fix it.

Format:
1) Brief explanation of the bug
2) Corrected function in a Python code block

Here is the function:

def sum_of_evens(numbers):
    total = 0
    for num in numbers:
        if num % 2 == 1:
            total += num
    return total

**AI's Corrected Code:**
```python

def sum_of_evens(numbers):
    total = 0
    for num in numbers:
        if num % 2 == 0:
            total += num
    return total


That’s it — **do not put code inside yet**.

---

## Save
Press **Ctrl + S**

---

### ✅ STOP
Do not add anything else.

Reply with **exactly**:

**step 5 done**

Then we’ll add the corrected code in the next step.

**What I Learned:**
The bug was caused by checking for odd numbers instead of even numbers.


---

## Problem 2: Refactoring

**My Prompt:**




(that is: `**My Prompt:**`, then an empty code block with triple backticks)

---

## Save
Press **Ctrl + S**

---

### ✅ STOP
Do **not** add anything else yet.

Reply with **exactly**:

**problem 2 step 2 done**

Then I’ll give you **Problem 2 Step 3**.



You are a senior Python developer who writes clean, Pythonic code.

Context:
The following function works correctly, but it is written in a verbose
and non-idiomatic way.

Task:
Refactor the function to make it clearer, more concise, and more Pythonic
while keeping the same behavior.

Format:
Return the refactored function in a Python code block.

Here is the function:

def get_names_of_adults(users):
    results = []
    for i in range(len(users)):
        if users[i]['age'] >= 18:
            results.append(users[i]['name'])
    return results

**AI's Refactored Code:**
```python
def get_names_of_adults(users):
    return [user["name"] for user in users if user["age"] >= 18]


---

## Save
Press **Ctrl + S**

---

### ✅ STOP
Reply with **exactly**:

**problem 2 step 4 done**

Then we’ll paste the refactored function.

**What I Learned:**
Using list comprehensions and iterating directly over the list makes code more readable and Pythonic.

---

## Problem 3: Documenting
**My Prompt:**

You are a Python developer and technical writer.

Context:
The following function works correctly, but it does not have any documentation.
It raises a ValueError if the inputs are not positive.

Task:
Write a professional NumPy-style docstring for this function.

Format:
Return the function with the new NumPy-style docstring included.

Here is the function:

def calculate_area(length, width):
    if length <= 0 or width <= 0:
        raise ValueError("Length and width must be positive numbers.")
    return length * width


(that’s `**My Prompt:**` and then an empty triple-backtick block)

---

## Save
Press **Ctrl + S**

---

### ✅ STOP
Do **not** add anything else yet.

Reply with **exactly**:

**problem 3 step 2 done**

Then we’ll add the actual prompt text next.

**AI's Documented Code:**
```python
def calculate_area(length, width):
    """Calculate the area of a rectangle.

    Parameters
    ----------
    length : float or int
        The length of the rectangle. Must be a positive number.
    width : float or int
        The width of the rectangle. Must be a positive number.

    Returns
    -------
    float or int
        The area of the rectangle.

    Raises
    ------
    ValueError
        If length or width is less than or equal to zero.
    """
    if length <= 0 or width <= 0:
        raise ValueError("Length and width must be positive numbers.")
    return length * width

**What I Learned:**
Writing NumPy-style docstrings clearly explains how a function should be used, what it returns, and what errors it may raise.

