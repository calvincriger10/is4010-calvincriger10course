## Problem 1: Finding common items

**Prompt:**
I have two very large lists of product IDs and need to find which IDs appear in both lists. Order does not matter and performance is important. Which Python data structure should I use and why?

**AI Recommendation & Reasoning:**
Use sets. Sets provide very fast membership testing (O(1)) and support intersection operations. Converting both lists to sets and using a set intersection efficiently finds common elements and automatically removes duplicates.


## Problem 2: User profile lookup

**Prompt:**
I have a list of user dictionaries (name, age, email). I frequently need to look up a user by username and performance is critical. What Python data structure should I use?

**AI Recommendation & Reasoning:**
Use a dictionary keyed by username. Dictionaries allow O(1) lookup time, which is much faster than searching through a list each time. Converting the list into a dictionary with usernames as keys makes repeated lookups efficient.


## Problem 3: Listing even numbers in order

**Prompt:**
I have a list of integers and need to return only the even numbers while preserving their original order. Which Python data structure or approach should I use?

**AI Recommendation & Reasoning:**
Use a list with a list comprehension. Lists preserve order, and a list comprehension provides a concise and efficient way to filter values while maintaining the original sequence.
