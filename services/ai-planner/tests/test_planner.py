"""Tests for planner agent."""

import unittest
import sys
import os

# Add the src directory to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from agents.planner_agent import PlannerAgent

class TestPlannerAgent(unittest.TestCase):
    def test_agent_initialization(self):
        """Test that the planner agent can be initialized."""
        agent = PlannerAgent()
        self.assertIsInstance(agent, PlannerAgent)
    
    def test_plan_infrastructure(self):
        """Test infrastructure planning."""
        agent = PlannerAgent()
        requirements = {
            "compute": {
                "cpu": "4 cores",
                "memory": "8 GB"
            }
        }
        result = agent.plan_infrastructure(requirements)
        self.assertEqual(result['status'], 'success')
    
    def test_optimize_plan(self):
        """Test plan optimization."""
        agent = PlannerAgent()
        result = agent.optimize_plan({})
        self.assertEqual(result['status'], 'success')

if __name__ == '__main__':
    unittest.main()
