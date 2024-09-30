# Team Isidore GitHub Rules and Coding Conventions

## Coding Conventions

For this project, we mainly used the Rust programming language for backend, as well as the combination of extended HTML and Javascript in `.svelte` files for frontend.

### Linter Rules
To help keep our code consistent with our decided conventions, we make use of Super-Linter, which supplies linters for every language used in this project and more. Checks are performed on every Pull Request into our default branch.

To help us abide by these rules, we use Prettier with most things kept default in order to match the configurations of the linters in the workflow.
``` JSON
{
  trailingComma: "all",
  tabWidth: 2,
  semi: true,
  singleQuote: false,
  bracketSameLine: false,
}
```
### Rust

With Rust specifically, we decided to stick with the recommended configuration of the Rustfmt linter, enforcing the '[Default Rust Style](https://doc.rust-lang.org/nightly/style-guide/)'.  
We only enabled the newer 2021 Rust linter available on Super-Linter, since we made use of some of the latest features including those surrounding hyperthreading, and it suited our needs very well.

## GitHub Rules

### Branches

We intend to follow git flow for our branching strategy, with two core branches:

- main
  - Merging to this branch deploys to our production droplet. Plenty of tests and quality checks are performed before our code reaches this point.
  - We make use of a staging branch to prepare and test our deployment strategy. Once our testing succeeds and we are certain everything is ready, we prepare to merge into main.
- dev
  - Default developing branch, this is where everything from all other branches come together.

### Which branch do I push to?

When we start working on a new feature, we always create a new branch with the corresponding "feature/" prefix, as specified below. It is acceptable to make "umbrella" branches, which can suit the implemented of multiple features, or can be further branched off of.  
The main branch should never be pushed to directly, and the same can be said for the dev branch. In all cases, we push our changes to branches that branch off of these main branches, and then later merge our code as described below, which will trigger our important workflow checks.

### How does my code merge into the core branches?

When you are done implementing a feature/fix, submit a pull request. On these requests we enforce the rule that at least one (or two people, depending on the branch) review the changes before the merge may happen.
Every pull request also triggers automated checks to run, such as our tests, linter and build procedures, to ensure that only working and convention-abiding code becomes part of the actual project.

### Branch naming conventions

Every branch name starts with a branch category, followed by a descriptive name.
Possible Branch Categories:

- feature/
- fix/
- setup/
- docs/

The specific branch names will be all lower case, and hyphen separated.
For example:

- feature/view-dashboard
- fix/grid-visualisation-bug
