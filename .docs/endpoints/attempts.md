# Attempts Endpoints

## GET `/attempts/course/{courseId}/quiz/{quizId}`
- Summary: List attempts for a quiz (management view).
- Auth: Bearer token required.
- Roles (AuthzAction::AttemptList): `admin`, `func`, `student`.
- Policy: requires course manager membership (`func` or `assistant`) or `admin`.
- Path params:
  - `courseId: uuid`
  - `quizId: uuid`
- Response `data`: `AttemptListItemView[]`
  - `{ attemptId, studentId, userName, quizId, startedAt, expiresAt, submittedAt, resultsViewedAt, score, grade }`

## POST `/attempts/quiz/{quizId}`
- Summary: Initialize an attempt for current user.
- Auth: Bearer token required.
- Roles (AuthzAction::AttemptInitialize): `admin`, `func`, `student`.
- Policy: quiz must exist, started, not closed, and user must not have a previous attempt.
- Path params:
  - `quizId: uuid`
- Body: none.
- Response `data`: `AttemptView`
  - `{ attemptId, quizId, startedAt, expiresAt, submittedAt, resultsViewedAt, questions }`
  - `questions[]`: `{ id, prompt, options, images }`

## PUT `/attempts/{attemptId}/answers/{questionId}`
- Summary: Save or update one answer in attempt.
- Auth: Bearer token required.
- Roles (AuthzAction::AttemptSubmit): `admin`, `func`, `student`.
- Policy: attempt must belong to current user, be open, not submitted, and question must belong to attempt.
- Path params:
  - `attemptId: uuid`
  - `questionId: uuid`
- Body:

```json
{
  "answerIndex": 1,
  "certaintyLevel": "low"
}
```

- Notes:
  - `answerIndex >= 0`.
  - `certaintyLevel` required for `certainty`, forbidden for `traditional`.
- Response `data`: none (success envelope only).

## POST `/attempts/{attemptId}/submit`
- Summary: Submit an attempt and persist score/grade.
- Auth: Bearer token required.
- Roles (AuthzAction::AttemptSubmit): `admin`, `func`, `student`.
- Policy: attempt must belong to current user, not expired, and not already submitted.
- Path params:
  - `attemptId: uuid`
- Body: none.
- Response `data`: `AttemptSubmitView`
  - `{ attemptId, quizId, submittedAt }`

## GET `/attempts/join/{joinCode}/results/me`
- Summary: View own attempt result by join code (student-facing).
- Auth: Bearer token required.
- Roles (AuthzAction::QuizViewAttemptResultByCode): `admin`, `func`, `student`.
- Policy: student can only view after quiz results are published, and only once unless the quiz has `resultsMultipleViews`.
- Path params:
  - `joinCode: string`
- Response `data`: `AttemptResultView`
  - `{ attemptId, studentId, userName, quizId, submittedAt, grade, score, maxScore, resultsViewedAt, questions }`
  - `questions[]`: `{ questionId, question, options, images, answerIndex, correctAnswerIndex, justification, certaintyLevel, isCorrect, awardedPoints }` (`justification`: Markdown/LaTeX o `null`)

## GET `/attempts/{attemptId}/results/managed`
- Summary: View evaluated attempt result (management view).
- Auth: Bearer token required.
- Roles (AuthzAction::AttemptViewResultsManaged): `admin`, `func`, `student`.
- Policy: requester must be course manager.
- Path params:
  - `attemptId: uuid`
- Response `data`: `AttemptResultView`
  - `{ attemptId, studentId, userName, quizId, submittedAt, grade, score, maxScore, resultsViewedAt, questions }`
  - `questions[]`: `{ questionId, question, options, images, answerIndex, correctAnswerIndex, justification, certaintyLevel, isCorrect, awardedPoints }` (`justification`: Markdown/LaTeX o `null`)

## Common Domain Errors
- `403`: quiz not started, quiz ended, expired, already submitted, forbidden.
- `404`: attempt not found.
- `400`: invalid answer index, certainty rule violation, question not in attempt.
