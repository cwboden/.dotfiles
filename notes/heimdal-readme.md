# How to Update Heimdal
If you're reading this, sorry in advance for the journey on which you are about to partake.  I have
tried to detail the steps necessary to update the `.patch`es so that you can avoid stumbling through
the same steps as I did.  Let's jump in!

# Quick Command
Hate reading? Use this command to get started:

```bash
# Clone via Git
git clone {url}
cd heimdal/
git checkout {branch}
rm -rf .git/

# Initialize Mercurial
hg init
hg add .
hg commit -m "Heimdal v{version}"
hg init --mq
hg qimport ~/src/toolchain/packages/heimdal/*.patch
```

# Cloning the Repo
First you will need to clone the version of Heimdal you wish to update to (`git clone {url}`). In
the past, this has been from [Heimdal's own repo](https://github.com/heimdal/heimdal), but you also
may need to clone from a [consultant's repo](https://github.com/chapeltech/heimdal), like I did.
Afterwards, move into the directory where we'll be working (`cd heimdal/`).

Whichever you use, you will then want to checkout the specific branch, if necessary (`git checkout
{branch_name}`) and finally delete the `.git/` directory (`rm -rf .git/`), since we won't be using
Git's version control for this, as our `.patch` files are based on Mercurial.

If you're interested in updating this guide to work natively in Git, please do. It would probably
simplify the workflow.

# Setting Up Mercurial
We'll then need to initialize Mercurial to import the existing `.patch` files (`hg init`). Make sure
to then add the existing files as an "initial" patch so that our changes can be applied on top of
them (`hg add .; hg commit -m "Heimdal v{version}`)

Next, start up the Mercurial Queues so that we can import our patches (`hg init --mq; hg qimport
{patches}`). Note that you may not need to `qimport` all of the patches -- some may have been
upstreamed and merged into Heimdal already.

# Applying and Fixing Patches
Now that the patches are imported, we can apply them to the repo and fix any merge conflicts or
behavior changes so that they can be applied within our toolchain correctly.

One by one, push patches on top of the newest version, making any changes necessary. Unfortunately,
there's not a great guide for this -- the changes will depend on what's been updated in Heimdal.

Once you are satisfied with your work, copy the patches back into
`~/src/toolchain/packages/heimdal/`, overwriting the existing patches. Nice work, you have now
upgraded Heimdal! Follow the standard guide for toolchain updates to push your changes into `~/src`.
Don't forget to create a new `.tar.gz` of the latest version of Heimdal (without your patch changes)
on Gravytrain so that the toolchain is able to download the new version.

You'll likely want to run some tests to verify that everything is working as intended.
`auth/kerberos/auth_context_test.c` is a good place to start, though things may have shifted by the
time you're reading this.
