initSidebarItems({"enum":[["ReferencingFields","The scalar fields on the concrete side relation."],["RefinedRelationWalker","Splits the relation to different types."],["RelationName","The relation name."]],"struct":[["CompleteInlineRelationWalker","Represents a relation that has fields and references defined in one of the relation fields. Includes 1:1 and 1:n relations that are defined from both sides."],["CompositeTypeFieldWalker","A field in a composite type."],["CompositeTypeWalker","A composite type, introduced with the `type` keyword in the schema."],["DefaultValueWalker","An `@default()` attribute on a field."],["FieldWalker","A model field, scalar or relation."],["ImplicitManyToManyRelationWalker","Describes an implicit m:n relation between two models. Neither side defines fields, attributes or referential actions, which are all inferred by Prisma."],["IndexWalker","An index, unique or fulltext attribute."],["InferredField","A scalar inferred by loose/magic reformatting"],["InlineRelationWalker","An explicitly defined 1:1 or 1:n relation. The walker has the referencing side defined, but might miss the back relation in the AST."],["ModelWalker","A `model` declaration in the Prisma schema."],["PrimaryKeyWalker","An `@(@)id` attribute in the schema."],["RelationFieldWalker","A relation field on a model in the schema."],["RelationWalker","A relation that has the minimal amount of information for us to create one. Useful for validation purposes. Holds all possible relation types."],["ScalarFieldAttributeWalker","A scalar field as referenced in a key specification (id, index or unique)."],["ScalarFieldWalker","A scalar field, as part of a model."],["Walker","A generic walker. Only walkers intantiated with a concrete ID type (`I`) are useful."]],"type":[["EnumValueWalker","One value in an `enum` declaration in the schema."],["EnumWalker","An `enum` declaration in the schema."]]});