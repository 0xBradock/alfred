# Alfred

> Track tasks and goals on the terminal

Yet another attempt to keep track the things I have to do, in the terminal.

# Features

- Keep track of goals with its percentage of execution
- Display a "burn down" for my current goals when the terminal loads
- Display activities for `today`, `tomorrow`, `this week` and `this month`
- Display if goal is `on-time` or `overdue`
- Display tasks ordered by `due_datetime`

# Activity: Goal or Task

Think of **goal** as something to be accomplished before a date and that you can track the progress over time.
Ex: Read 12 books in a year, one leetcode a day, etc.

Goal is composed of:

- *`id`: The goal uuid
- *`name`: The goal name
- *`tracking`: Frequency of tracking in number of days
- *`due_date`: Date of goal
- `url`: Web information
- `notes`: Any further information concerning the goal

Think of **task** as a one-off activity in a specific date.
Ex: Dentist, travel, going out for dinner with someone, taxes, etc.

Task is composed of:

- *`id`: The task uuid
- *`name`: The task name
- *`due_datetime`: Date and time of task
- *`location`: Addres where task take place
- `context`: If the task pertains to a specific area (sport, family, work, friends, etc.)
- `travel_time`: Estimation of the time to get where task wil take place
- `url`: Web information
- `notes`: Any further information concerning the task

> [!NOTE]
> \* Required fields