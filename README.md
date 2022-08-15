# NI-DAQmx-Sys

This crate provides direct bindings for the DAQmx C API.

These are generated using the bindgen CLI as provided in the powershell script. This is currently configured to be run on a Windows System with NI DAQmx C support installed.

## Versioning

I'm concerned about how to do this without too much confusion.

The policy is:

1. Use the same version as the DAQmx version used to generate the bindings.
2. If they need other updates, use the bugfix version.

Older version should be compatible if NI do not introduced breaking changes. However they do not follow a semver policy so this may be difficult to follow.