# prop2sql Compiler

A compiler that transforms propositional logic expressions into SQL queries.

## Specification

### Input Format
The compiler receives a string with the following format:
`($table) : ($prop)`

- `($table)`: The name of the database table to query.
- `($prop)`: A propositional logic expression.

### Output
The compiler returns a SQL query that selects all elements from the specified table where the given property evaluates to true.

```sql
SELECT * FROM ($table) WHERE ($sql_condition);
```

### Operators
- **Conjunction (AND)**: `[Aa][Nn][Dd] &[&?]`
- **Disjunction (OR)**: `[Oo][Rr], |[|?]`
- **Negation (NOT)**: `!` `[Nn][Oo][[Tt]?]`
