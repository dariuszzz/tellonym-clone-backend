# tellonym-clone-backend
Api for https://github.com/dariuszzz/tellonym-clone

## Requirements:
- min rust 1.65 nightly
- mysql database named `tellonym_clone` on localhost
- .env with DATABASE_URL and ACCESS_SECRET

# Types 

## user 
```
json {
  id: int,
  username: string,
  follower_coount: int,
  following_count: int, //amount of people followed by this user
  bio: string,
}
```

## question 
```
json {
  question: { 
    id: int,
    content: string,
    likes: int,
    asked_id: int,
    asked_at: iso8601 date string,
    asker_id: int | null, //null if the question was anonymous 
  },
  answer: {
    id: int,
    question_id: int,
    content: string,
    likes: int,
    answered_at: iso8601 date string,
    last_edit_at: iso8601 date string, //the same as `answered_at` if it wasn't edited
  } | null
}
```

# Routes

- GET `/users [?search=<string>]` <br>
  <= `json [ user1: user, user2: user, ... ]` <br>
  [ ?search=Foo will returns users with usernames starting with "Foo"]

- GET `/users/<user_id>` <br>
  <= `json user` <br>
  returns the user with the specified id

- GET `/me` <br>
  <= `json user` <br>
  requires access token <br>
  returns the user logged in with the access token

- POST `/register` <br>
  => `json { username: string, password: string }` <br>
  <= access token (plaintext), refresh token in cookie

- POST `/login` <br>
  => `json { username: string, password: string }` <br>
  <= access token (plaintext), refresh token in cookie

- POST `/refres` <br>
  <= access token (plaintext) <br>
  requires refresh token in cookies

- POST `/users/<user_id>/ask` <br>
  => `json { anonymous: bool, content: string }` <br>
  requires access token <br>
  add a question to the specified user (ask them)

- GET `/users/<user_id>/questions` <br>
  <= `json [ question1: question, question2: question, ... ]` <br>
  returns the list of questions which were sent that user

- POST `/questions/<question_id>/answer` <br>
  => `json { content: string }` <br>
  requires access token of the user with `id` equal to the `asked_id` of the question being answered<br>
  answers a specific question

- GET `/questions/<question_id>` <br>
  <= `json question` <br>
  returns the question with given id

