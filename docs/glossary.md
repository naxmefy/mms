# Glossary

Terms used throughout mms; the term is also the name used in code.

| Term | Definition | Status |
|---|---|---|
| Article | A product that is managed in stock, identified by its SKU. | v0.1 |
| Batch | A quantity of an article produced or received together and tracked as one unit, e.g. for recalls. | later |
| Best-before date | The date until which an article or batch keeps its specified properties. | later |
| Goods issue | A stock movement that removes a quantity of an article from a location. | v0.1 |
| Goods receipt | A stock movement that adds a received quantity of an article to a location. | v0.1 |
| Location | A place where stock is kept; locations form a tree, from a whole warehouse down to a single bin. | v0.1 |
| Master data | Long-lived reference data such as articles and locations, created, updated and deactivated. | v0.1 |
| Physical inventory | Counting the actual stock and recording differences as stock adjustments. | later |
| Reversal | A stock movement that cancels an earlier one; movements are never edited or deleted. | v0.1 |
| Serial number | A unique identifier for a single item of an article. | later |
| SKU | Stock keeping unit: the unique identifier of an article. | v0.1 |
| Stock | Quantity of an article at a location, derived from stock movements. | v0.1 |
| Stock adjustment | A stock movement that corrects stock to match reality, e.g. after damage or a count. | v0.1 |
| Stock movement | An immutable record of a quantity of an article entering, leaving or moving between locations. | v0.1 |
| Stock transfer | A stock movement that moves a quantity of an article from one location to another. | v0.1 |
| Transaction data | Records of business events, such as stock movements, that are appended and never changed. | v0.1 |
| Unit of measure | The unit in which a quantity is expressed, e.g. piece, kilogram or metre. | v0.1 |
| Warehouse | A top-level location, typically a building or site. | v0.1 |
| Workspace | A folder containing an mms data set (`.mms/` with configuration and database). | v0.1 |
