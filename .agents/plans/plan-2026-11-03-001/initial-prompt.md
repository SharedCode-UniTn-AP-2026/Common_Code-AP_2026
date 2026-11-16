Port the previously developed Line of Sight implementation from the old Geometry repository into the current Geometry crate.

The existing implementation includes visible(), its helper functions, and the associated tests.

The goal of this task is to restore the previous implementation as a starting point for further development by @alterax05 .

Scope:

Copy the existing Line of Sight implementation into the current crate.
Preserve the existing tests where possible.
Apply only the minimal adjustments required for compatibility with the current Cell and Map APIs.
