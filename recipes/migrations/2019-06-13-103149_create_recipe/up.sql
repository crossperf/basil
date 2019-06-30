Create Table recipe (
  recipe_id Serial Primary Key,
  category Text Not Null,
  slug Text Not Null,
  title Text Not Null,
  image Text Not Null,
  description Text Not Null,
  calories Integer Not Null,
  protein Integer Not Null,
  fat Integer Not Null,
  gluten_free Boolean Not Null,
  egg_free Boolean Not Null
);

Create Table recipe_ingredient (
  recipe_ingredient_id Serial Primary Key,
  recipe_id Integer Not Null References recipe(recipe_id),
  ingredient Text Not Null,
  amount Numeric(12, 2) Not Null,
  unit Text,
  seq Integer Not Null
);

Create Table recipe_direction (
  recipe_direction_id Serial Primary Key,
  recipe_id Integer Not Null References recipe(recipe_id),
  title Text Not Null,
  description Text Not Null,
  image Text,
  seq Integer Not Null
);
