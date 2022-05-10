# Immediate challenges
In migrating a midsize (12,000 LOC incl. tests) Ruby backend to Rust, a couple of immediate
challenges present.

First, given that the aim of this migration is to eliminate data-level errors in non-local
environments, the removal of non-necessary database columns seemed as good a place as any to start.
That said, determining redundant database columns is easier said than done.

We can `rake db:structure:dump` the Ruby app to get the schema in SQL which helps a bit, given that
Diesel requires migrations in SQL. We could also reverse-engineer the existing database but
that would mean we can't recreate the environment in a straightforward manner. Plus, it wouldn't
shed the redundant columns anyway. So far I'm just grepping parts of `structure.sql` and
hand-selecting what is required and what isn't.

```
rg 'CREATE TABLE public.users' structure.sql -A 52
```

About as good as we're going to get here.

Second, and speaking of structure, I don't want ~3,000 lines of Ruby controller files should be
in one `api.rs`. I guess one model-controller combination per database table is tolerable.

## First efforts
We're sticking with Postgres, and we'll need an ORM. Diesel appears to be the de facto standard so
let's keep it simple. We'll use Warp as the HTTP server since I've used it in a microservice
recently and it affords a decent amount of flexibility without too much macro magic or anything of
the sort.
