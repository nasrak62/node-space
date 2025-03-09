# TODO:

1. symlink sync (check which symlink are active)
   a. as part of regular commands
   b. new command
   c. for a whole group

2. watch coordinator
   a. listen for messages
   b. add file watchers for project and dependencies
   c. when file change (dedup events), run the build command and then trigger build
   for all of the projects which are dependent on this one

   d. move all prints to log files

3. watch coordinator client
   a. check if coordinator is active if not start it
   b. register project and add watchers (can be direct watcher in the client or at least stream the result)
   if the project has dependencies register them as well - at the coordinator
   c. close coordinator

4. serve html

5. install packages (package manager)

6. bundle files
