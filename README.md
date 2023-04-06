# tellonym-clone-backend
Api for https://github.com/dariuszzz/tellonym-clone

## Requirements:
- min rust 1.65 nightly
- mysql database named `tellonym_clone` on localhost
- .env with DATABASE_URL, REFRESH_SECRET and ACCESS_SECRET

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

## like
```
json {
  id: number,
  liker_id: number,
  like_type: string, //QuestionLike | QuestionDislike | AnswerLike | AnswerDislike
  resource_id: number, //question id if Question(Dis)Like and answer id if Answer(Dis)like
}
```

# Routes
if required the access token needs to be in a Authorization header prefixed with "Bearer: "

- GET `/users [?search=<string>]` <br>
  <= `json [ user1: user, user2: user, ... ]` <br>
  [ ?search=Foo will returns users with usernames starting with "Foo"]

- GET `/users/<user_id>` <br>
  <= `json user` <br>
  returns the user with the specified id

- GET `/me` <br>
  <= `json { user, likes: [ like1: like, like2: like, ... ] }` <br>
  returns the user logged in with the access token along with his likes <br>
  requires access token

- POST `/register` <br>
  => `json { username: string, password: string }` <br>
  <= access token (plaintext), refresh token in cookie
  username has to be at least 3 characters long <br>
  password has to be at least 8 characters long

- POST `/login` <br>
  => `json { username: string, password: string }` <br>
  <= access token (plaintext), refresh token in cookie

- POST `/refresh` <br>
  <= access token (plaintext) <br>
  requires refresh token in cookies

- POST `/users/<user_id>/ask` <br>
  => `json { anonymous: bool, content: string }` <br>
  requires access token <br>
  add a question to the specified user (ask them)

- GET `/users/<user_id>/questions` <br>
  <= `json [ question1: question, question2: question, ... ]` <br>
  returns the list of questions which were sent that user

- GET `/users/<user_id>/follows` <br>
  <= `json [ user1: user, user2: user, ...]` <br>
  returns a list of users followed by the user with user_id

- GET `/users/<user_id>/followers` <br>
  <= `json [ user1: user, user2: user, ...]` <br>
  returns a list of users who follow the user with user_id

- POST `/users/<user_id>/follow` <br>
  requires access token <br>
  follows the given user, or unfollows if the logged in user was already following them

- POST `/questions/<question_id>/answer` <br>
  => `json { content: string }` <br>
  requires access token of the user with `id` equal to the `asked_id` of the question being answered <br>
  answers a specific question

- GET `/questions/<question_id>` <br>
  <= `json question` <br>
  returns the question with given id

- POST `/questions/<question_id>/vote_question` <br>
  => `json { is_like: bool }` <br>
  votes on the question with question_id <br>
  is_like determines whether the vote is a like (true) or a dislike (false) <br>
  removes the vote if it already exists and changes it if the opposite already exists (ie dislike -> like, like -> dislike) <br>
  requires access token

- POST `/questions/<question_id>/vote_answer` <br>
  => `json { is_like: bool }` <br>
  votes on the answer of the question with question_id <br>
  is_like determines whether the vote is a like (true) or a dislike (false) <br>
  removes the vote if it already exists and changes it if the opposite already exists (ie dislike -> like, like -> dislike) <br>
  requires access token

- POST `/editprofile` <br>
  <= `multipart { username?: string, current_pass?: string, password?: string, bio?: string, profile_pic?: file` <br>
  requires access token <br>
  sets whatever you pass (you stay logged in even if changing pass) <br>
  if password is set then current_password has to be set as well (and equal to previous password)

- GET `/pfps/<id>.png` <br>
  returns pfp of user with id `<id>` 
  if the user does not have a pfp this returns the default pfp (at /pfps/0.jpg)

- GET `/homepage` <br>
  <= `json [ { question, answer }, { question, answer } ]`
  returns questions answered by people followed by the current user <br>
  requires access token