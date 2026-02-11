/**
 * Quiz scoring matrix - maps each question's choice to character weights
 * Each row corresponds to a question, each object in the row to a choice
 */
export const scoring = [
  [
    { tank: 0, carla: 0, crosby: 2 },
    { tank: 1, carla: 1, crosby: 0 },
    { tank: 0, carla: 2, crosby: 1 },
    { tank: 1, carla: 0, crosby: 1 }
  ],
  [
    { tank: 1, carla: 0, crosby: 1 },
    { tank: 0, carla: 2, crosby: 0 },
    { tank: 0, carla: 1, crosby: 1 },
    { tank: 2, carla: 0, crosby: 0 }
  ],
  [
    { tank: 2, carla: 0, crosby: 0 },
    { tank: 0, carla: 2, crosby: 0 },
    { tank: 0, carla: 0, crosby: 2 },
    { tank: 1, carla: 1, crosby: 0 }
  ],
  [
    { tank: 1, carla: 0, crosby: 1 },
    { tank: 0, carla: 2, crosby: 0 },
    { tank: 2, carla: 0, crosby: 0 },
    { tank: 0, carla: 1, crosby: 1 }
  ],
  [
    { tank: 1, carla: 0, crosby: 1 },
    { tank: 0, carla: 2, crosby: 0 },
    { tank: 2, carla: 0, crosby: 0 },
    { tank: 0, carla: 0, crosby: 2 }
  ],
  [
    { tank: 2, carla: 0, crosby: 0 },
    { tank: 0, carla: 2, crosby: 0 },
    { tank: 0, carla: 0, crosby: 2 },
    { tank: 1, carla: 1, crosby: 0 }
  ],
  [
    { tank: 2, carla: 0, crosby: 0 },
    { tank: 0, carla: 2, crosby: 0 },
    { tank: 0, carla: 0, crosby: 2 },
    { tank: 1, carla: 1, crosby: 0 }
  ],
  [
    { tank: 1, carla: 0, crosby: 1 },
    { tank: 0, carla: 2, crosby: 0 },
    { tank: 2, carla: 0, crosby: 0 },
    { tank: 0, carla: 1, crosby: 1 }
  ]
];
