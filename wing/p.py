  def Process(self, elapsed):
        pAVar3 = ((AttackBehavior)self).attack
        tower = pAVar3.super.tower,
        a = tower.targetType.fields.id
        b = self.GetName()
        if a != b:
            return
        lastElapsed = self.lastElapsed
        self.lastElapsed = elapsed
        wingmonkeyPatternModel = self.wingmonkeyPatternModel
        delta = (elapsed - lastElapsed) * 0.01666667
        if wingmonkeyPatternModel.useGrouping:
            wingmonkeySimulation = self.wingmonkeySimulation
            isLeader = wingmonkeySimulation.IsLeader(wingmonkeySimulation,self)
            if not isLeader:
                wingmonkeySimulation = self.wingmonkeySimulation
                positionInGroup = Assets.Scripts.Simulation.SimulationBehaviors.WingmonkeySimulation$$GetPositionInGroup(local_28,wingmonkeySimulation,self)
                self.ApplyMovement(positionInGroup, delta)
                return
        fVar14 = self.timeUntilNextCheckForTargets - delta
        self.timeUntilNextCheckForTargets = fVar14
        if not self.flyoverEngaged:
            if (not self.wasTargetValidLastFrame or
                self.currentBloonTarget and not ((5x super)self.currentBloonTarget).isDestroyed
            ):
                if not self.currentBloonTarget or ((5x super)self.currentBloonTarget).isDestroyed:
                    positionInGroup = self.GetRandomFlyoverPosition(local_28,self)
                    self.flyoverDestination = positionInGroup
                    goto LAB_18054e088
            else:
                pAVar12 = self.GetBloonTarget(true)
                self.currentBloonTarget = pAVar12
                if not pAVar12:
                    self.flyoverDestination = lastPositionOfTarget

LAB_18054e088:
                    self.flyoverEngaged = true
        elif self.canCheckForTargetsWhileInFlyover and fVar14 <= 0.0:
            if self.currentBloonTarget and not ((5x super)self.currentBloonTarget).isDestroyed
                self.flyoverEngaged = false
                self.wasTargetValidLastFrame = true
                self.canCheckForTargetsWhileInFlyover = false
                wingmonkeyPatternModel = self.wingmonkeyPatternModel
                self.timeUntilNextCheckForTargets = wingmonkeyPatternModel.updateDelay
            if not self.currentBloonTarget or ((5x super)self.currentBloonTarget).isDestroyed:
                self.wasTargetValidLastFrame = false
            self.lastPositionOfTarget.fields = 0
            self.UpdateMovement(delta)
            return
        self.wasTargetValidLastFrame = true
        pAVar13 = Assets.Scripts.Simulation.Objects.CommonBehaviorProxy$$get_Position(pAVar12),
        self.lastPositionOfTarget = pAVar13
        self.UpdateMovement(delta)
        return