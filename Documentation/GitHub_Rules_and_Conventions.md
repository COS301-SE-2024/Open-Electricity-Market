# Team Isidore GitHub Rules and Conventions

## Coding Conventions

### Linter Rules


### Code Documentation


## GitHub Rules

### Branches

We intend to follow git flow for our branching strategy, with two core branches:
- main
	- Default branch, this is where the stable releases will be.
- dev
	- Developing branch, this is where everything comes together.

### Which branch do I push to?
When we start working on a new feature, we always create a new branch with the corresponding "feature/" prefix, as specified below. It is acceptable to make "umbrella" branches, which can suit the implemented of multiple features, or can be further branched off of.  
The main branch should never be pushed to directly, and the same can be said for the dev branch, with some small exceptions such as extremely small updates before major milestones, or that need to get on all branches quickly.  

### How does my code merge into the core branches?
When you are done implementing a feature/fix, submit a pull request. 

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