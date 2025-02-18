def UpdateMovement(self, delta):
        zerovector = Assets.Scripts.Simulation.SMath.Vector3.zero
        local_68.fields.[xy] = zerovector.fields.[xy];
        if not self.currentBloonTarget or ((5x super)self.currentBloonTarget).isDestroyed:
            uVar9 = zerovector.fields.[xy]
            if self.flyoverEngaged:
                uVar9.x = self.flyoverDestination.fields.x
                uVar9.y = self.flyoverDestination.fields.y
                zerovector.fields.z = self.flyoverDestination.fields.z
        else:
            wingmonkeyPatternModel = self.wingmonkeyPatternModel,
            _path = targetBloon._path
            dist_overshoot = targetBloon.super.radius * 0.5 + wingmonkeyPatternModel.overshootDistance
            fVar17 = dist_overshoot + targetBloon.distanceTraveled
            bVar10 = Assets.Scripts.Simulation.Track.Path$$IsInBoundsOfPath(_path,fVar17)
            targetBloon = self.currentBloonTarget
            # check if overshoot distance + distance already travelled would be off the path
            if bVar10: # if within path
                pAVar11 = targetBloon._path.DistanceToPoint(_path,fVar17)
            else:
                pAVar14 = targetBloon.get_Position()
                targetBloon = self.currentBloonTarget
                pUVar12 = pAVar11 * dist_overshoot
                pAVar11 = pAVar14 + pUVar12
            zerovector.fields.x = pAVar11.x
            zerovector.fields.y = pAVar11.y
            local_68.fields.z = pAVar11.z
            local_68.fields.[xy] = zerovector.fields.[xy]
            pAVar14 = self.airUnit.transform.position,
            zerovector.fields.z = pAVar14.???
            lhs = Unity.Mathematics.float3$$get_xy(&zerovector)
            AVar13 = self.airUnit.transform.position.ToVector2()
            local_res8 = lhs - AVar13
            dist_overshoot = Assets.Scripts.Simulation.SMath.Vector2$$get_MagnitudeSquared(local_res8)
            uVar9 = zerovector.fields.[xy]
            if dist_overshoot <= 1600.0:
                AVar13 = Assets.Scripts.Simulation.SMath.Vector2$$Normalized(local_res8)
                targetBloon = self.currentBloonTarget
                pAVar14 = targetBloon.get_Position()
                self.flyoverDestination = uVar2 = pAVar14
                local_res20 = AVar13 * 90.0
                pAVar11 = Assets.Scripts.Simulation.SMath.Vector2$$ToVector3(local_res20)
                local_78 = pAVar11
                local_68 = uVar2
                pAVar11 = local_68 + local_78
                self.flyoverDestination = pAVar11
                self.flyoverEngaged = true
                self.currentBloonTarget = None
        zerovector.fields.[xy] = uVar9
        pAVar14 = self.airUnit.get_Position()
        zerovector.fields.z = pAVar14.???
        local_68 = zerovector
        self.ApplyMovement(&local_68, delta)
        pAVar14 = self.airUnit.transform.position,
        local_68 = zerovector
        dist_overshoot = pAVar14.Distance(local_68)
        if dist_overshoot <= 30.0:
            targetBloon = self.GetBloonTarget(false)
            self.currentBloonTarget = targetBloon
            if not self.currentBloonTarget or ((5x super)self.currentBloonTarget).isDestroyed
                pAVar11 = self.GetRandomFlyoverPosition()
                wingmonkeyPatternModel = self.wingmonkeyPatternModel;
                self.flyoverDestination = pAVar11
                self.flyoverEngaged = true
                self.wasTargetValidLastFrame = false
                self.canCheckForTargetsWhileInFlyover = true
                self.timeUntilNextCheckForTargets = wingmonkeyPatternModel.updateDelay
            else:
                self.flyoverEngaged = false
                self.wasTargetValidLastFrame = true
                self.canCheckForTargetsWhileInFlyover = false