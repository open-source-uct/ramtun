# Quizzes Endpoints

## GET `/quizzes/{quizId}`
- Summary: Get one managed quiz.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizReadManaged): `admin`, `func`, `student`.
- Policy: user must have access to the quiz context.
- Path params:
  - `quizId: uuid`
- Response `data`: `QuizView`

## GET `/quizzes/me`
- Summary: List quizzes managed by current user.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizListManaged): `admin`, `func`, `student`.
- Response `data`: `QuizView[]`

## POST `/quizzes`
- Summary: Create quiz from course + selected banks.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizCreate): `admin`, `func`, `student`.
- Policy: user must be allowed in course and bank context.
- Body:

```json
{
  "courseId": "uuid",
  "title": "Control Semana 3",
  "kind": "traditional",
  "startsAt": "2026-05-10T15:00:00Z",
  "attemptDurationMinutes": 30,
  "questionCount": 10,
  "bankIds": ["uuid"],
  "certaintyConfig": null,
  "resultsMultipleViews": false
}
```

- Notes:
  - `kind=certainty` requires `certaintyConfig`.
  - `kind=traditional` must not include `certaintyConfig`.
  - `resultsMultipleViews` (optional, default `false`): lets students reopen their results; otherwise they can view them once.
- Response `data`: `QuizView`

## DELETE `/quizzes/{quizId}`
- Summary: Soft-delete quiz.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizDeleteManaged): `admin`, `func`, `student`.
- Path params:
  - `quizId: uuid`
- Response `data`: none (success envelope only).

## POST `/quizzes/{quizId}/close-and-publish`
- Summary: Close quiz and publish results in one step.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizCloseAndPublishManaged): `admin`, `func`, `student`.
- Path params:
  - `quizId: uuid`
- Response `data`: none (success envelope only).

## POST `/quizzes/join/{joinCode}`
- Summary: Resolve join code and return join preview.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizJoinByCode): `admin`, `func`, `student`.
- Path params:
  - `joinCode: string`
- Body: none.
- Response `data`: `JoinQuizPreviewView`
  - `{ quizId, title, kind, questionCount, certaintyTable, attemptDurationMinutes, startsAt }`

## Output Shape: `QuizView`
- `{ id, title, kind, joinCode, questionCount, certaintyTable, attemptDurationMinutes, startsAt, resultsPublishedAt, resultsMultipleViews, createdAt, course }`
- `course` is embedded course entity data.

## Common Domain Errors
- `404`: quiz not found.
- `403`: forbidden quiz access.
- `400`: invalid create payload or mode mismatch.
- `409`: quiz closed or locked by existing attempts.
