# `prasiolite-bb` design plan

things i would like to add:

* the site is a list of *forums*; a forum is a list of *topics*; a topic is a list of *posts*
  * optional: the site is instead a list of *categories*, where a category is a list of forums
* posts are written in markdown and rendered before saving to db
  * at some point i'll need to figure out whether i want to add or remove things from the markdown
    spec i use
  * at minimum i'll want to make sure i scrub the text i accept from users; cf. work from
    docs.rs/crates.io
    * note to self: the `comrak` and `ammonia` crates are what you want here
* posts are authored by user accounts
  * user accounts are saved in browser sessions
  * users may be invited or may register themselves
  * a user account has a *username*, at minimum; other profile data is optional
  * users need some way to authenticate; email/password? OAuth forwarding? user certs?
    * if i keep an email, do i need to be able to *send* emails? "click a link in an email to sign
      up" is a frequent anti-spam measure
  * some sites may be invite-only, requiring a generated code/link to create an account
* some users are moderators or admins, and need powers to do more than just write or edit posts
  * moderators have powers to shape their community
    * they can "scrub" others' posts (hiding them behind a click? removing them entirely?)
    * they can "timeout" or "ban" user accounts (preventing them from posting/viewing for a duration
      or forever)
    * they can invite new users
    * they can "lock" a thread (preventing non-mods from posting there)
  * admins have power to customize the site's internals
    * they have all mod powers
    * they can customize site structure; site/forum titles, add/remove forums
    * they can "lock" a forum (preventing new topics from being created)
  * some forums may only be visible to mods/admins
* the site is rendered with a series of templates, which serve as the site theme
  * multiple themes may be packed by default; e.g. dark/light themes
  * optional: templates may be modified by users, or sets of them may be offered as "themes"
* running the server may have certain options set via config file or CLI flags
  * e.g. database name/URL, binding hostname/port
* site config and posts are stored in a database, likely postgres
