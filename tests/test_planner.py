"""Tests for planner agent."""

import unittest
import sys
import os

# Add the src directory to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from agents.planner_agent import PlannerAgent
from ai_planner.decision_engine import DecisionEngine
from ai_planner.cost_predictor import CostPredictor
from ai_planner.provider_selector import ProviderSelector
from ai_planner.memory_manager import MemoryManager

class TestPlannerAgent(unittest.TestCase):
    def test_agent_initialization(self):
        """Test that the planner agent can be initialized."""
        agent = PlannerAgent()
        self.assertIsInstance(agent, PlannerAgent)
        self.assertIsInstance(agent.decision_engine, DecisionEngine)
        self.assertIsInstance(agent.cost_predictor, CostPredictor)
        self.assertIsInstance(agent.provider_selector, ProviderSelector)
        self.assertIsInstance(agent.memory_manager, MemoryManager)
    
    def test_plan_infrastructure(self):
        """Test infrastructure planning."""
        agent = PlannerAgent()
        requirements = {
            "resource_type": "compute",
            "cpu": 4,
            "memory": 8,
            "region": "us-east-1"
        }
        result = agent.plan_infrastructure(requirements)
        self.assertEqual(result['status'], 'success')
        self.assertIn('plan', result)
        self.assertIn('context_key', result)
    
    def test_optimize_plan(self):
        """Test plan optimization."""
        agent = PlannerAgent()
        plan = {
            "decision": {"resource_type": "compute"},
            "provider_selection": {"selected_provider": "aws"}
        }
        result = agent.optimize_plan(plan)
        self.assertEqual(result['status'], 'success')
        self.assertIn('optimized_plan', result)
        self.assertIn('recommendations', result)
    
    def test_decision_engine(self):
        """Test decision engine."""
        engine = DecisionEngine()
        requirements = {"resource_type": "compute"}
        result = engine.make_decision(requirements)
        self.assertEqual(result['status'], 'approved')
    
    def test_cost_predictor(self):
        """Test cost predictor."""
        predictor = CostPredictor()
        config = {"type": "compute", "cpu": 4, "memory": 8, "hours": 24}
        result = predictor.predict_cost(config)
        self.assertIn('estimated_cost', result)
    
    def test_provider_selector(self):
        """Test provider selector."""
        selector = ProviderSelector()
        requirements = {"region": "europe"}
        result = selector.select_provider(requirements)
        self.assertIn('selected_provider', result)
    
    def test_memory_manager(self):
        """Test memory manager."""
        manager = MemoryManager()
        test_data = {"key": "value"}
        manager.store_context("test_key", test_data)
        retrieved = manager.retrieve_context("test_key")
        self.assertEqual(retrieved, test_data)

if __name__ == '__main__':
    unittest.main()
