#[doc = "Register `SCL_STRETCH_CONF` reader"]
pub type R = crate::R<SCL_STRETCH_CONF_SPEC>;
#[doc = "Register `SCL_STRETCH_CONF` writer"]
pub type W = crate::W<SCL_STRETCH_CONF_SPEC>;
#[doc = "Field `STRETCH_PROTECT_NUM` reader - Configure the period of I2C slave stretching SCL line."]
pub type STRETCH_PROTECT_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `STRETCH_PROTECT_NUM` writer - Configure the period of I2C slave stretching SCL line."]
pub type STRETCH_PROTECT_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `SLAVE_SCL_STRETCH_EN` reader - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when reg_slave_scl_stretch_en is 1 and stretch event happens. The stretch cause can be seen in reg_stretch_cause."]
pub type SLAVE_SCL_STRETCH_EN_R = crate::BitReader;
#[doc = "Field `SLAVE_SCL_STRETCH_EN` writer - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when reg_slave_scl_stretch_en is 1 and stretch event happens. The stretch cause can be seen in reg_stretch_cause."]
pub type SLAVE_SCL_STRETCH_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLAVE_SCL_STRETCH_CLR` writer - Set this bit to clear the I2C slave SCL stretch function."]
pub type SLAVE_SCL_STRETCH_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLAVE_BYTE_ACK_CTL_EN` reader - The enable bit for slave to control ACK level function."]
pub type SLAVE_BYTE_ACK_CTL_EN_R = crate::BitReader;
#[doc = "Field `SLAVE_BYTE_ACK_CTL_EN` writer - The enable bit for slave to control ACK level function."]
pub type SLAVE_BYTE_ACK_CTL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLAVE_BYTE_ACK_LVL` reader - Set the ACK level when slave controlling ACK level function enables."]
pub type SLAVE_BYTE_ACK_LVL_R = crate::BitReader;
#[doc = "Field `SLAVE_BYTE_ACK_LVL` writer - Set the ACK level when slave controlling ACK level function enables."]
pub type SLAVE_BYTE_ACK_LVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:9 - Configure the period of I2C slave stretching SCL line."]
    #[inline(always)]
    pub fn stretch_protect_num(&self) -> STRETCH_PROTECT_NUM_R {
        STRETCH_PROTECT_NUM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when reg_slave_scl_stretch_en is 1 and stretch event happens. The stretch cause can be seen in reg_stretch_cause."]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&self) -> SLAVE_SCL_STRETCH_EN_R {
        SLAVE_SCL_STRETCH_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - The enable bit for slave to control ACK level function."]
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&self) -> SLAVE_BYTE_ACK_CTL_EN_R {
        SLAVE_BYTE_ACK_CTL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set the ACK level when slave controlling ACK level function enables."]
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&self) -> SLAVE_BYTE_ACK_LVL_R {
        SLAVE_BYTE_ACK_LVL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_STRETCH_CONF")
            .field(
                "stretch_protect_num",
                &format_args!("{}", self.stretch_protect_num().bits()),
            )
            .field(
                "slave_scl_stretch_en",
                &format_args!("{}", self.slave_scl_stretch_en().bit()),
            )
            .field(
                "slave_byte_ack_ctl_en",
                &format_args!("{}", self.slave_byte_ack_ctl_en().bit()),
            )
            .field(
                "slave_byte_ack_lvl",
                &format_args!("{}", self.slave_byte_ack_lvl().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_STRETCH_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Configure the period of I2C slave stretching SCL line."]
    #[inline(always)]
    #[must_use]
    pub fn stretch_protect_num(&mut self) -> STRETCH_PROTECT_NUM_W<SCL_STRETCH_CONF_SPEC, 0> {
        STRETCH_PROTECT_NUM_W::new(self)
    }
    #[doc = "Bit 10 - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when reg_slave_scl_stretch_en is 1 and stretch event happens. The stretch cause can be seen in reg_stretch_cause."]
    #[inline(always)]
    #[must_use]
    pub fn slave_scl_stretch_en(&mut self) -> SLAVE_SCL_STRETCH_EN_W<SCL_STRETCH_CONF_SPEC, 10> {
        SLAVE_SCL_STRETCH_EN_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear the I2C slave SCL stretch function."]
    #[inline(always)]
    #[must_use]
    pub fn slave_scl_stretch_clr(&mut self) -> SLAVE_SCL_STRETCH_CLR_W<SCL_STRETCH_CONF_SPEC, 11> {
        SLAVE_SCL_STRETCH_CLR_W::new(self)
    }
    #[doc = "Bit 12 - The enable bit for slave to control ACK level function."]
    #[inline(always)]
    #[must_use]
    pub fn slave_byte_ack_ctl_en(&mut self) -> SLAVE_BYTE_ACK_CTL_EN_W<SCL_STRETCH_CONF_SPEC, 12> {
        SLAVE_BYTE_ACK_CTL_EN_W::new(self)
    }
    #[doc = "Bit 13 - Set the ACK level when slave controlling ACK level function enables."]
    #[inline(always)]
    #[must_use]
    pub fn slave_byte_ack_lvl(&mut self) -> SLAVE_BYTE_ACK_LVL_W<SCL_STRETCH_CONF_SPEC, 13> {
        SLAVE_BYTE_ACK_LVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Set SCL stretch of I2C slave\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stretch_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stretch_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_STRETCH_CONF_SPEC;
impl crate::RegisterSpec for SCL_STRETCH_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_stretch_conf::R`](R) reader structure"]
impl crate::Readable for SCL_STRETCH_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_stretch_conf::W`](W) writer structure"]
impl crate::Writable for SCL_STRETCH_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_STRETCH_CONF to value 0"]
impl crate::Resettable for SCL_STRETCH_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
